# This code is what was used to generate main.c.

# (function scm_main ()
#   (function sum (acc pos vec)
#     (if (== (vector_length vec) pos)
#       acc
#       (sum (+ acc (vector_ref vec pos)) (+ pos 1) vec)))
#   (commit_integer (sum 0 0 (read_vector))))

[:function, :scm_main, [],
    [:function, :sum, [:acc, :pos, :vec],
      [:if, [:==, [:vector_length, :vec], :pos],
        :acc,
        [:sum, [:+, :acc, [:vector_ref, :vec, :pos]], [:+, :pos, 1], :vec]]],
    [:commit_integer, [:sum, 0, 0, [:read_vector]]]]
