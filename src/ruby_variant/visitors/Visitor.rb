module Visitor
    def visit_integertype(node)
        raise NotImplementedError
    end

    def visit_floattype(node)
        raise NotImplementedError
    end

    def visit_booleantype(node)
        raise NotImplementedError
    end

    def visit_stringtype(node)
        raise NotImplementedError
    end

    def visit_celladdress(node)
        raise NotImplementedError
    end

    def visit_add(node)
        raise NotImplementedError
    end

    def visit_subtract(node)
        raise NotImplementedError
    end

    def visit_multiply(node)
        raise NotImplementedError
    end

    def visit_divide(node)
        raise NotImplementedError
    end

    def visit_modulo(node)
        raise NotImplementedError
    end

    def visit_exponent(node)
        raise NotImplementedError
    end

    def visit_negate(node)
        raise NotImplementedError
    end

    def visit_land(node)
        raise NotImplementedError
    end

    def visit_lor(node)
        raise NotImplementedError
    end

    def visit_lnot(node)
        raise NotImplementedError
    end

    def visit_band(node)
        raise NotImplementedError
    end

    def visit_bor(node)
        raise NotImplementedError
    end

    def visit_bxor(node)
        raise NotImplementedError
    end

    def visit_bnot(node)
        raise NotImplementedError
    end

    def visit_lshift(node)
        raise NotImplementedError
    end

    def visit_rshift(node)
        raise NotImplementedError
    end

    def visit_eq(node)
        raise NotImplementedError
    end

    def visit_neq(node)
        raise NotImplementedError
    end

    def visit_lt(node)
        raise NotImplementedError
    end

    def visit_lteq(node)
        raise NotImplementedError
    end

    def visit_gt(node)
        raise NotImplementedError
    end

    def visit_gteq(node)
        raise NotImplementedError
    end

    def visit_function(node)
        raise NotImplementedError
    end

end
