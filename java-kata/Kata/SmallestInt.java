package Kata;

import java.util.stream.IntStream;

public class SmallestInt {
    public static int findSmallestInt(int[] n) {
        return IntStream.of(n).min().getAsInt();
    }
}
