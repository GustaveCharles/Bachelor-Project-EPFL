; ModuleID = 'hello'
source_filename = "hello"

@message = external global [13 x i8]

define void @main() {
entry:
  store [13 x i8] c"Hello, World!", ptr @message, align 1
  %puts = call i32 @puts([13 x i8] c"Hello, World!")
  ret void
}

declare i32 @puts(ptr)
