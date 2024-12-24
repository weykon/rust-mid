/// 当我去定义一棵树的时候，遇到了
/// struct TreeNode { 
///     value: i32, 
///     left: Option<Box<TreeNode>>,
///     right: Option<Box<TreeNode>>
/// }
/// 和
/// struct TreeNode { 
///     value: i32, 
///     left: Option<Rc<RefCell<TreeNode>>>,
///     right: Option<Rc<RefCell<TreeNode>>>
/// }
/// 的选择，我们可以从这个选择中去理解，使用box和rcrefcell的区别
/// 这是结论：
/// 当系统复杂度增加时，Box 的单一所有权模型可能会成为限制，
/// 而 Rc<RefCell> 的"细胞式"管理能够更好地支持复杂的业务需求。
/// 尽管有一些运行时开销，但这个代价相对于获得的灵活性和可维护性来说是值得的。
/// 那么举个例子： 
/// 假设这是一个文件系统的节点
 
// 
#[derive(Debug)]
struct FileNode {
    name: String,
    content: String,
    // 其他元数据...
}

// 使用Box的版本 - 严格的树形结构
#[derive(Debug)]
struct BoxFileSystem {
    root: Box<FileNode>,
    // 想要建立额外的索引会很困难
    // 因为每个节点都被其父节点独占所有权
}

// 使用Rc<RefCell>的版本 - 灵活的数据管理
#[derive(Debug)]
struct RcFileSystem {
    root: Rc<RefCell<FileNode>>,
    // 可以轻松维护额外的索引和缓存
    name_index: HashMap<String, Rc<RefCell<FileNode>>>,
    content_cache: HashMap<String, Vec<Rc<RefCell<FileNode>>>>,
}


// 1. 首先看Box版本中所有权传递的问题
#[derive(Debug)]
struct BoxFile {
    name: String,
    content: String,
    children: Vec<Box<BoxFile>>,
}

impl BoxFile {
    fn new(name: &str) -> Self {
        BoxFile {
            name: name.to_string(),
            content: String::new(),
            children: vec![],
        }
    }

    // 要修改深层节点，需要层层传递可变引用
    fn update_deep_file(&mut self, path: &[String], new_content: String) {
        if path.is_empty() {
            self.content = new_content;
            return;
        }

        for child in &mut self.children {
            if child.name == path[0] {
                // 递归调用，传递剩余路径
                child.update_deep_file(&path[1..], new_content);
                return;
            }
        }
    }
}

// 使用Box的问题示例
fn box_example() {
    let mut root = BoxFile::new("root");
    let mut docs = BoxFile::new("docs");
    docs.children.push(Box::new(BoxFile::new("file1.txt")));
    root.children.push(Box::new(docs));

    // 要修改深层文件，需要从根节点开始获取可变引用
    root.update_deep_file(
        &vec!["docs".to_string(), "file1.txt".to_string()],
        "new content".to_string()
    );
}

// 2. 使用Rc<RefCell>实现观察者模式
type Callback = Box<dyn Fn(&str)>;

struct RcFile {
    name: String,
    content: String,
    children: Vec<Rc<RefCell<RcFile>>>,
    // 观察者列表
    observers: Vec<Callback>,
}

impl RcFile {
    fn new(name: &str) -> Self {
        RcFile {
            name: name.to_string(),
            content: String::new(),
            children: vec![],
            observers: vec![],
        }
    }

    // 添加观察者
    fn add_observer(&mut self, callback: Callback) {
        self.observers.push(callback);
    }

    // 更新内容并通知观察者
    fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        // 通知所有观察者
        for observer in &self.observers {
            observer(&self.content);
        }
    }
}

fn rc_example() {
    // 创建文件节点
    let file = Rc::new(RefCell::new(RcFile::new("example.txt")));
    
    // 添加多个观察者
    {
        let mut file_mut = file.borrow_mut();
        file_mut.add_observer(Box::new(|content| {
            println!("Observer 1: Content changed to {}", content);
        }));
        file_mut.add_observer(Box::new(|content| {
            println!("Observer 2: Content changed to {}", content);
        }));
    }

    // 在不同的地方持有同一个文件的引用
    let file_ref1 = Rc::clone(&file);
    let file_ref2 = Rc::clone(&file);

    // 可以在任何地方更新文件内容
    file_ref1.borrow_mut().update_content("Hello".to_string());
    file_ref2.borrow_mut().update_content("World".to_string());
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
// 3. 异步场景下的示例 (使用Arc<Mutex>)
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct AsyncFile {
    name: String,
    content: String,
}

fn async_example() {
    let file = Arc::new(Mutex::new(AsyncFile {
        name: "async.txt".to_string(),
        content: String::new(),
    }));

    let mut handles = vec![];

    // 创建多个线程同时操作文件
    for i in 0..3 {
        let file_clone = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file = file_clone.lock().unwrap();
            file.content.push_str(&format!("Thread {} wrote this\n", i));
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最终结果
    println!("Final content: {:?}", file.lock().unwrap().content);
}

fn main() {
    println!("Box example:");
    box_example();

    println!("\nRc<RefCell> example with observers:");
    rc_example();

    println!("\nAsync example with Arc<Mutex>:");
    async_example();
}
