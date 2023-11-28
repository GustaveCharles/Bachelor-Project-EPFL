; ModuleID = 'sum'
source_filename = "sum"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

define i64 @main(i64 %0, i64 %1, i64 %2) {
entry:
  %sum = add i64 %0, %1
  %sum1 = add i64 %sum, %2
  ret i64 %sum1
}
