; ModuleID = 'hello-translation'
source_filename = "hello-translation"

@my_global_var = external global [0 x i8]

define i32 @main(i32 %0, i32 %1) {
entry:
  %"%R0_0" = alloca i32, align 4
  %"%R1_0" = alloca i32, align 4
  %"%R2_0" = alloca i32, align 4
  %"%R3_0" = load i32, ptr %"%R0_0", align 4
  %cmpeq = icmp ne i32 %"%R3_0", 0
  br i1 %cmpeq, label %after_end1, label %else

after_end:                                        ; preds = %else5, %else
  %"%R5_07" = load i32, ptr %"%R2_0", align 4
  ret i32 %"%R5_07"

after_end1:                                       ; preds = %entry
  br label %loop

else:                                             ; preds = %entry
  %"%R3_02" = load i32, ptr %"%R1_0", align 4
  store i32 %"%R3_02", ptr %"%R2_0", align 4
  br label %after_end

loop:                                             ; preds = %loop, %after_end1
  %"%R4_0" = load i32, ptr %"%R1_0", align 4
  %"%R4_03" = load i32, ptr %"%R0_0", align 4
  store i32 %"%R4_03", ptr %"%R2_0", align 4
  %"%4" = urem i32 %"%R4_0", %"%R4_03"
  store i32 %"%4", ptr %"%R0_0", align 4
  %"%R5_0" = load i32, ptr %"%R2_0", align 4
  store i32 %"%R5_0", ptr %"%R1_0", align 4
  %"%R5_04" = load i32, ptr %"%R0_0", align 4
  %cmpeq6 = icmp ne i32 %"%R5_04", 0
  br i1 %cmpeq6, label %loop, label %else5

else5:                                            ; preds = %loop
  br label %after_end
}
