package Kata;

import java.util.stream.IntStream;

public class MinMax {
    public static int min(int[] a) {
        return IntStream.of(a).min().orElse(0);
    }
    public static int max(int[] a) {
        return IntStream.of(a).max().orElse(0);
    }
}
