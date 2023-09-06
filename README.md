# Day3 CloudWeGo Practice 

## 测试

`cargo test`

## 注意

我们必须使用unsafe：因为这样的内存访问pattern天然违反了所有权系统。  
UnsafeCell唯一的正面用处是绕过对裸指针的限制，没有实际意义，所以并未采用。

## 广告

个人博客-从偏序关系和泛型系统的角度理解生命周期：[https://cubicy.icu/2023/09/05/rust-lifetime/](https://cubicy.icu/2023/09/05/rust-lifetime/)