; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@format_str_s_ = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@format_str_d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@format_str_s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@argc_global = global i32 0
@argv_global = global ptr null
@global_string = global [5 x i8] c"hello"

declare i32 @printf(i8*, ...)

define i32 @printd(i32 %num) {
entry:
  %format_str = getelementptr inbounds [4 x i8], [4 x i8]* @format_str_d, i32 0, i32 0
  %calltmp = call i32 (i8*, i32) @printf(i8* %format_str, i32 %num)
  ret i32 %num
}

define i32 @enter(i32 %0, ptr %1) {
entry:
  %calltmp1 = call i32 @printd(i32 3)
  call i32 @main()
  ret i32 0
}

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 %0, 5
  ret i32 %addtmp
}

define i32 @main() {
  %calltmp = call i32 @hello(i32 3)
  %calltmp1 = call i32 @printd(i32 %calltmp)
  ret i32 0
}
