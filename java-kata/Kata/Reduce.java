package Kata;

import java.util.stream.IntStream;

public class Reduce {
    public static int grow(int[] x) {
        return IntStream.of(x).reduce((a, i) -> a * i).getAsInt();
    }
}
