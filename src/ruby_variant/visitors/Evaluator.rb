class Evaluator
    include Visitor

    def visit_integertype(node)
        node.value
    end

    def visit_floattype(node)
        node.value
    end

    def visit_booleantype(node)
        node.value
    end

    def visit_stringtype(node)
        node.value
    end

    def visit_celladdress(node)
        raise NotImplementedError
    end

    def visit_add(node)
        node.left.accept(self) + node.right.accept(self)
    end

    def visit_subtract(node)
        node.left.accept(self) - node.right.accept(self)
    end

    def visit_multiply(node)
        node.left.accept(self) * node.right.accept(self)
    end

    def visit_multiply(node)
        node.left.accept(self) / node.right.accept(self)
    end

    def visit_modulo(node)
        node.left.accept(self) % node.right.accept(self)
    end

    def visit_exponent(node)
        node.left.accept(self) ** node.right.accept(self)
    end

    def visit_negate(node)
        -node.operand.accept(self)
    end

    def visit_land(node)
        node.left.accept(self) && node.right.accept(self)
    end

    def visit_lor(node)
        node.left.accept(self) || node.right.accept(self)
    end

    def visit_lnot(node)
        !node.operand.accept(self)
    end

    def visit_band(node)
        node.left.accept(self) & node.right.accept(self)
    end

    def visit_bor(node)
        node.left.accept(self) | node.right.accept(self)
    end

    def visit_bxor(node)
        node.left.accept(self) ^ node.right.accept(self)
    end

    def visit_bnot(node)
        ~node.operand.accept(self)
    end

    def visit_lshift(node)
        node.left.accept(self) << node.right.accept(self)
    end

    def rshift(node)
        node.left.accept(self) >> node.right.accept(self)
    end

    def visit_eq(node)
        node.left.accept(self) == node.right.accept(self)
    end

    def visit_neq(node)
        node.left.accept(self) != node.right.accept(self)
    end

    def visit_lt(node)
        node.left.accept(self) < node.right.accept(self)
    end

    def visit_lteq(node)
        node.left.accept(self) <= node.right.accept(self)
    end

    def visit_gt(node)
        node.left.accept(self) > node.right.accept(self)
    end

    def visit_gteq(node)
        node.left.accept(self) >= node.right.accept(self)
    end

    def visit_function(node)
        # fti, itf, max, min, mean, sum
        raise NotImplementedError
    end
end