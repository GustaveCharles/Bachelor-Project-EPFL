; ModuleID = 'hello-translation'
source_filename = "hello-translation"

@my_global_var = external global [0 x i8]
@memory = global [100 x i8] c"\08\00\00\00\0C\00\00\00hello world\0A\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"

declare i32 @fd_write(i32, i32, i32, i32)

define void @main() {
entry:
  store i32 8, ptr getelementptr inbounds (ptr, ptr @memory, i32 8), align 4
  store i32 8, ptr @memory, align 4
  store i32 12, ptr getelementptr inbounds (ptr, ptr @memory, i32 12), align 4
  store i32 12, ptr getelementptr inbounds (ptr, ptr @memory, i32 4), align 4
  %"%F0" = call i32 @fd_write(i32 1, i32 0, i32 1, i32 20)
  ret void
}
