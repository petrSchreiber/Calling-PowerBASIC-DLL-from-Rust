' STRINGZ = ASCIIZ
sub passing_stringz_ptr alias "passing_stringz_ptr" (byVal szp as stringz ptr) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, @szp)
end sub

sub passing_stringz_ptr_with_change alias "passing_stringz_ptr_with_change" (byVal szp as stringz ptr) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, @szp)
  @szp = "Hi, Rust!"
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, @szp)
end sub

sub passing_stringz_byref alias "passing_stringz_byref" (byref sz as stringz) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, sz)
end sub
