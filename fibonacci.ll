; ModuleID = 'hello-translation'
source_filename = "hello-translation"

@my_global_var = external global [0 x i8]
@"%G0" = global i32 65592

declare void @proc_exit(i32)

declare i32 @fd_write(i32, i32, i32, i32)

define void @"%F2"() {
entry:
  ret void
}

define i32 @"%F3"(i32 %0) {
entry:
  %"%R0_3" = alloca i32, align 4
  store i32 %0, ptr %"%R0_3", align 4
  %"%R1_3" = alloca i32, align 4
  %"%R2_3" = alloca i32, align 4
  %"%R3_3" = load i32, ptr %"%R0_3", align 4
  %"3" = icmp slt i32 %"%R3_3", 2
  br i1 %"3", label %then, label %ifcont

then:                                             ; preds = %entry
  %"%R3_31" = load i32, ptr %"%R0_3", align 4
  ret i32 %"%R3_31"

ifcont:                                           ; preds = %entry
  br label %loop

loop:                                             ; preds = %loop, %ifcont
  %"%R4_3" = load i32, ptr %"%R0_3", align 4
  %"%4" = sub i32 %"%R4_3", 1
  %"%F3" = call i32 @"%F3"(i32 %"%4")
  %"%R5_3" = load i32, ptr %"%R1_3", align 4
  %"%5" = add i32 %"%F3", %"%R5_3"
  store i32 %"%5", ptr %"%R1_3", align 4
  %"%R6_3" = load i32, ptr %"%R0_3", align 4
  %"6" = icmp ugt i32 %"%R6_3", 3
  %"%R6_32" = load i32, ptr %"%R0_3", align 4
  %"%6" = sub i32 %"%R6_32", 2
  store i32 %"%6", ptr %"%R0_3", align 4
  br i1 %"6", label %loop, label %else

else:                                             ; preds = %loop
  %"%R7_3" = load i32, ptr %"%R0_3", align 4
  %"%R7_33" = load i32, ptr %"%R1_3", align 4
  %"%7" = add i32 %"%R7_3", %"%R7_33"
  ret i32 %"%7"
}

define void @main() {
entry:
  %"%R0_4" = alloca i32, align 4
  %"%R1_4" = alloca i32, align 4
  %"%F3" = call i32 @"%F3"(i32 10)
  %"%G0" = load i32, ptr @"%G0", align 4
  %"%2" = sub i32 %"%G0", 16
  store i32 %"%2", ptr %"%R0_4", align 4
  store i32 %"%2", ptr @"%G0", align 4
  %"%R3_4" = load i32, ptr %"%R0_4", align 4
  %cast = zext i32 %"%R3_4" to i64
  %add_offset = add i64 12, %cast
  %add_ptr = add i64 %add_offset, ptrtoint (ptr @memory to i64)
  %ptr_build = inttoptr i64 %add_ptr to ptr
  store i32 18, ptr %ptr_build, align 4
  %"%R3_41" = load i32, ptr %"%R0_4", align 4
  %cast2 = zext i32 %"%R3_41" to i64
  %add_offset3 = add i64 8, %cast2
  %add_ptr4 = add i64 %add_offset3, ptrtoint (ptr @memory to i64)
  %ptr_build5 = inttoptr i64 %add_ptr4 to ptr
  store i32 1024, ptr %ptr_build5, align 4
  %"%R3_46" = load i32, ptr %"%R0_4", align 4
  %"%3" = add i32 %"%R3_46", 8
  %"%R4_4" = load i32, ptr %"%R0_4", align 4
  %"%4" = add i32 %"%R4_4", 4
  %"%F1" = call i32 @fd_write(i32 1, i32 %"%3", i32 1, i32 %"%4")
  store i32 %"%F1", ptr %"%R1_4", align 4
  %cmpeq = icmp ne i32 %"%F1", 0
  br i1 %cmpeq, label %then, label %ifcont

then:                                             ; preds = %entry
  %"%R5_4" = load i32, ptr %"%R1_4", align 4
  store i32 %"%R5_4", ptr inttoptr (i64 add (i64 ptrtoint (ptr @memory to i64), i64 1044) to ptr), align 4
  br label %ifcont

ifcont:                                           ; preds = %then, %entry
  %"%R5_47" = load i32, ptr %"%R0_4", align 4
  %cast8 = zext i32 %"%R5_47" to i64
  %add_ptr9 = add i64 %cast8, ptrtoint (ptr @memory to i64)
  %ptr_build10 = inttoptr i64 %add_ptr9 to ptr
  %load = load i64, ptr %ptr_build10, align 4
  %"%R5_411" = load i32, ptr %"%R0_4", align 4
  %"%5" = add i32 %"%R5_411", 16
  store i32 %"%5", ptr @"%G0", align 4
  call void @proc_exit(i32 0)
  ret void
}

define i32 @"%F5"() {
entry:
  ret i32 1044
}

define i32 @"%F6"() {
entry:
  %"%G0" = load i32, ptr @"%G0", align 4
  ret i32 %"%G0"
}

define void @"%F7"(i32 %0) {
entry:
  %"%R0_7" = alloca i32, align 4
  store i32 %0, ptr %"%R0_7", align 4
  %"%R1_7" = load i32, ptr %"%R0_7", align 4
  store i32 %"%R1_7", ptr @"%G0", align 4
  ret void
}

define i32 @"%F8"(i32 %0) {
entry:
  %"%R0_8" = alloca i32, align 4
  store i32 %0, ptr %"%R0_8", align 4
  %"%G0" = load i32, ptr @"%G0", align 4
  %"%R1_8" = load i32, ptr %"%R0_8", align 4
  %"%1" = sub i32 %"%G0", %"%R1_8"
  store i32 -16, ptr %"%R0_8", align 4
  store i32 -16, ptr @"%G0", align 4
  %"%R2_8" = load i32, ptr %"%R0_8", align 4
  ret i32 %"%R2_8"
}