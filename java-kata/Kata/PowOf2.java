package Kata;

import java.util.stream.IntStream;

public class PowOf2 {
    public static long[] powersOfTwo(int n) {
        return IntStream.rangeClosed(0, n).mapToLong(e -> Math.round(Math.pow(2, e))).toArray();
    }
}
