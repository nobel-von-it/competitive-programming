package Kata;

import java.util.stream.IntStream;

public class SquareSum {
    public static int squareSum(int[] n) {
        return IntStream.of(n).map(e -> e*e).sum();
    }
}
