; ModuleID = 'hello'
source_filename = "hello"

@message = global [13 x i8] c"Hello, World!"

declare i32 @puts(ptr)

define void @main() {
entry:
  %puts_call = call i32 @puts(ptr @message)
  ret void
}
