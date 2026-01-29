package Kata;

import java.util.function.Function;

public class GetIterator {
    public static Function<Integer, Integer> getIterator(Function<Integer, Integer> f, int t) {
        Function<Integer, Integer> n = f;
        for (int i = 1; i < t; i++) {
            n = n.andThen(f);
        }
        return n;
    }
}
