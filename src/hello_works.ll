; ModuleID = 'branches_opti_shorter-translation'
source_filename = "branches_opti_shorter-translation"

@"%G0" = global i32 66592
@message = global [13 x i8] c"Hello, World!"

declare void @proc_exit(i32)

declare i32 @fd_write(i32, i32, i32, i32)

declare i32 @puts(ptr)

define void @"%F2"() {
entry:
  ret void
}

define i32 @"%F3"() {
entry:
  %"%F11" = call i32 @"%F11"(i32 13, i32 1024, i32 1)
  ret i32 0
}

define void @main() {
entry:
  br label %after_end

after_end:                                        ; preds = %else, %entry
  %"%F3" = call i32 @"%F3"()
  call void @"%F7"(i32 %"%F3")
  %puts_call = call i32 @puts(ptr @message)
  ret void

else:                                             ; preds = %entry
  call void @"%F2"()
  br label %after_end
}

define void @"%F5"() {
entry:
  ret void
}

define void @"%F6"() {
entry:
  %"%R0_6" = alloca i32, align 4
  store i32 0, ptr %"%R0_6", align 4
  br i1 true, label %after_end, label %else

after_end:                                        ; preds = %else, %entry
  call void @"%F5"()
  ret void

else:                                             ; preds = %else1, %loop, %entry
  br label %after_end

loop:                                             ; No predecessors!
  %"%R2_6" = load i32, ptr %"%R0_6", align 4
  %"%2" = add i32 %"%R2_6", -4
  store i32 %"%2", ptr %"%R0_6", align 4
  %"%R3_6" = load i32, ptr %"%R0_6", align 4
  %"3" = icmp ugt i32 0, %"%R3_6"
  br i1 %"3", label %else, label %else1

else1:                                            ; preds = %loop
  br label %else
}

define void @"%F7"(i32 %0) {
entry:
  %"%R0_7" = alloca i32, align 4
  call void @"%F5"()
  call void @"%F6"()
  call void @"%F5"()
  %"%R1_7" = load i32, ptr %"%R0_7", align 4
  call void @"%F8"(i32 %"%R1_7")
  ret void
}

define void @"%F8"(i32 %0) {
entry:
  %"%R0_8" = alloca i32, align 4
  %"%R1_8" = load i32, ptr %"%R0_8", align 4
  call void @proc_exit(i32 %"%R1_8")
  ret void
}

define i32 @"%F9"() {
entry:
  ret i32 1040
}

define i32 @"%F10"(i32 %0) {
entry:
  %"%R0_10" = alloca i32, align 4
  %"%R1_10" = load i32, ptr %"%R0_10", align 4
  %condition = icmp ne i32 %"%R1_10", 0  ; Check if %"%R1_10" is not equal to zero
  br i1 %condition, label %after_end, label %else

after_end:                                        ; preds = %else, %entry
  %"%F9" = call i32 @"%F9"()
  %"%R1_101" = load i32, ptr %"%R0_10", align 4
  ret i32 -1

else:                                             ; preds = %entry
  ret i32 0
  br label %after_end
}

define i32 @"%F11"(i32 %0, i32 %1, i32 %2) {
entry:
  %"%R0_11" = alloca i32, align 4
  %"%R1_11" = alloca i32, align 4
  %"%R2_11" = alloca i32, align 4
  %"%R3_11" = alloca i32, align 4
  %"%G0" = load i32, ptr @"%G0", align 4
  %"%4" = sub i32 %"%G0", 16
  store i32 %"%4", ptr %"%R3_11", align 4
  store i32 %"%4", ptr @"%G0", align 4
  %"%R5_11" = load i32, ptr %"%R3_11", align 4
  %"%R5_111" = load i32, ptr %"%R2_11", align 4
  %"%R5_112" = load i32, ptr %"%R3_11", align 4
  %"%R5_113" = load i32, ptr %"%R1_11", align 4
  %"%R5_114" = load i32, ptr %"%R0_11", align 4
  %"%R5_115" = load i32, ptr %"%R3_11", align 4
  %"%5" = add i32 %"%R5_115", 8
  %"%R6_11" = load i32, ptr %"%R3_11", align 4
  %"%6" = add i32 %"%R6_11", 4
  %"%F1" = call i32 @fd_write(i32 %"%6", i32 1, i32 %"%5", i32 %"%R5_114")
  %"%F10" = call i32 @"%F10"(i32 %"%F1")
  store i32 %"%F10", ptr %"%R2_11", align 4
  %"%R7_11" = load i32, ptr %"%R3_11", align 4
  store i32 %"%R7_11", ptr %"%R1_11", align 4
  %"%R7_116" = load i32, ptr %"%R3_11", align 4
  %"%7" = add i32 %"%R7_116", 16
  store i32 %"%7", ptr @"%G0", align 4
  %"%R8_11" = load i32, ptr %"%R1_11", align 4
  %"%R8_117" = load i32, ptr %"%R2_11", align 4
  ret i32 %"%R8_117"
}

define i32 @"%F12"() {
entry:
  %"%G0" = load i32, ptr @"%G0", align 4
  ret i32 %"%G0"
}

define void @"%F13"(i32 %0) {
entry:
  %"%R0_13" = alloca i32, align 4
  %"%R1_13" = load i32, ptr %"%R0_13", align 4
  store i32 %"%R1_13", ptr @"%G0", align 4
  ret void
}

define i32 @"%F14"(i32 %0) {
entry:
  %"%R0_14" = alloca i32, align 4
  %"%R1_14" = alloca i32, align 4
  %"%R2_14" = alloca i32, align 4
  %"%G0" = load i32, ptr @"%G0", align 4
  %"%R3_14" = load i32, ptr %"%R0_14", align 4
  %"%3" = sub i32 %"%G0", %"%R3_14"
  store i32 -16, ptr %"%R1_14", align 4
  store i32 -16, ptr @"%G0", align 4
  %"%R4_14" = load i32, ptr %"%R1_14", align 4
  ret i32 %"%R4_14"
}
