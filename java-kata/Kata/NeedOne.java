package Kata;

import java.util.Arrays;

public class NeedOne {
    public static boolean check(Object[] a, Object x) {
        for (Object o: a)
            if (o.equals(x))
                return true;
        return false;
    }
}
