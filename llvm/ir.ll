; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@format_str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i8 @printf(ptr, ...)

define i32 @printd(i32 %0) {
entry:
  %calltmp = call i8 (ptr, ...) @printf(ptr @format_str, i32 %0)
  ret i32 %0
}

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 %0, 5
  ret i32 %addtmp
}

define i32 @main(i32 %0, i32 %1) {
entry:
  %calltmp = call i32 @hello(i32 3)
  %calltmp1 = call i32 @printd(i32 %calltmp)
  ret i32 0
}