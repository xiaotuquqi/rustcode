/*王小波说：人在描述一切不熟悉的东西都会更加浪漫。
我想可能描述的就是我现在这种情况吧。
最近很忙，当有时间拿起来书去学习rust时候就是周一了，现在的能力大概只能勉强读懂代码（有java网络通信sokect的基础）实在是没有时间	去coding了，请谅解，下次一定好好完成！--刘宇开
*/

//客户端
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
//导入标准库
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{//定义函数 有定义参数、返回值和错误处理
    let mut buf = [0; 512];//定义数组大小是512 全是0
    for _ in 0..1000 {//1000次循环
        let bytes_read = stream.read(&mut buf)?;//从服务器读取数据
        if bytes_read == 0 {//如果数据是0 返回ok
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;//将读取的数据输入最初定义的数组
        thread::sleep(time::Duration::from_secs(1 as u64));//让该线程休眠1秒防止锁死
    }

    Ok(())
}

fn main() -> std::io::Result<()> {//主函数 返回结果
    let listener = TcpListener::bind("127.0.0.1:8080")?;//建立与该ip节点的连接
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");//错误处理
        let handle = thread::spawn(move || {//创造一个线程
            handle_client(stream)//应用上面定义的函数进行对服务器的数据读取
		.unwrap_or_else(|error| eprintln!("{:?}", error));//错误处理
        });
        thread_vec.push(handle);//将读取的数据放入定义的vector中
    }
    for handle in thread_vec {
        handle.join().unwrap();//待handle线程结束后运行unwrap
    }

    Ok(())
}

