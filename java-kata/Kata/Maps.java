package Kata;

import java.util.Arrays;
import java.util.stream.IntStream;

public class Maps {
    public static int[] map(int[] a) {
        return IntStream.of(a).map(e -> e*2).toArray();
    }
    public static int[] mapSimple(int[] a){
        for (int i = 0; i < a.length; i++) {
            a[i] *= 2;
        }
        return a;
    }
}
