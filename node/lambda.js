const C = c => () => c;
const I = () => i => i;

// the core def
const T = x => y => x;
const F = x => y => y;

const NOT = b => b(F)(T);
// const AND = b1 => b2 => b1(b2)(F);
const AND = b1 => b1(b2 => b2(T)(F))(F);
const OR = b1 => b2 => NOT(AND(NOT(b1))(NOT(b2)));


NOT(T) == F;
NOT(F) == T;
NOT(NOT(T)) == T;

AND(T)(T) == T;
AND(T)(NOT(T)) == F;