pub fn conversion() {
    // 将 &str 类型转为 String 类型
    {
        "hello,world".to_string();
    }
    // 将 String 类型转为 &str 类型
    {
        let s = String::from("hello,world!");
        say_hello(&s);
        say_hello(&s[..]);
        say_hello(s.as_str());
        fn say_hello(s: &str) {
            println!("{}", s);
        }
    }
}

// 追加 str
pub fn str_push() {
    // 字符串追加操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
    let mut s = String::from("Hello ");
    println!("{}", s);
    s.push_str("Rust");
    println!("{}", s);
    s.push('!');
    println!("{}", s);
}

// 插入 str
pub fn str_insert() {
    let mut s = String::from("HelloRust!");
    dbg!(&s);
    s.insert(5, ' ');
    dbg!(&s);
    s.insert_str(6, "一下插入很多内容");
    dbg!(&s);
}

// 替换 str
pub fn str_replace() {
    // 该方法是返回一个新的字符串，而不是操作原来的字符串。
    let initial = String::from("Hello rust! rust! rust! rust!");
    dbg!(&initial);


    // 第一个参数是要被替换的字符串，第二个参数是新的字符串
    // 适用于 String 和 &str
    let s1 = initial.replace("rust", "RUST");
    dbg!(&s1);


    // 第三个参数则表示替换的个数
    // 适用于 String 和 &str
    let s2 = initial.replacen("rust", "RUST", 2);
    dbg!(&s2);


    // 仅适用于 String
    // 直接操作原来的字符串  需要使用 mut 关键字修饰
    let mut s3 = String::from("I like rust!");
    s3.replace_range(7..8, "R");
    dbg!(&s3);
}

// 删除 str
pub fn str_delete() {
    // pop()，remove()，truncate()，clear()
    // 仅适用于 String 类型
    // 都是直接操作原字符串 需要mut修饰


    // pop
    // 返回值是一个 Option 类型，如果字符串为空，则返回 None。
    fn test_pop() {
        let mut string_pop = String::from("rust pop 中文!");
        dbg!(&string_pop);
        let p1 = string_pop.pop();
        dbg!(&p1);
        let p2 = string_pop.pop();
        dbg!(&p2);
        dbg!(&string_pop);
    }

    // remove
    // 一个参数，表示该字符起始索引位置
    // 返回值是删除位置的字符串
    fn test_remove() {
        let mut string_remove = String::from("测试remove方法");
        println!(
            "占 {} 个字节",
            std::mem::size_of_val(string_remove.as_str())
        );
        dbg!(&string_remove);
        // 删除第一个汉字
        let s1 = string_remove.remove(0);
        dbg!(&s1);

        // let r2 = string_remove.remove(1); // 报错  非法字符边界
        // 直接删除第二个汉字
        let r3 = string_remove.remove(3);
        dbg!(&r3);

        dbg!(&string_remove);
    }


    // truncate
    // 无返回值
    fn test_truncate() {
        let mut string_truncate = String::from("测试truncate");
        dbg!(&string_truncate);
        string_truncate.truncate(3);
        dbg!(&string_truncate);
    }

    // clear
    fn test_clear() {
        let mut string_clear = String::from("string clear");
        dbg!(&string_clear);
        string_clear.clear();
        dbg!(&string_clear);
    }
    test_clear()
}

// 连接
pub fn str_concatenate() {
    // 使用 + 和 += 连接
    {
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        // &string_rust会自动解引用为&str
        let result = string_append + &string_rust;
        let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
        result += "!!!";
        assert_eq!(result, "hello rust!!!!");


        // add会转移所有权
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
        let s3 = s1 + &s2;
        assert_eq!(s3, "hello,world!");
        // 此时再使用s1则会报错
        // println!("{}",s1);
    }

    // 使用 format! 连接字符串
    {
        let s1 = "hello";
        let s2 = String::from("rust");
        let s = format!("{} {}!", s1, s2);
        assert_eq!(s, "hello rust!")
    }
}

// 操作UTF-8字符
pub fn str_operation() {
    // 使用 chars 遍历字符串
    for c in "字符串".chars() {
        println!("{}", c); // 逐个输出字符
    }

    // 使用 bytes 遍历底层字节数组
    for b in "字符串".bytes() {
        println!("{}", b);
    }
}