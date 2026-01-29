package Kata;

import java.util.Arrays;
import java.util.stream.IntStream;

public class RemovEl {
    public static Object[] removeEveryOther(Object[] a) {
        int l = a.length, count = 0;
        Object[] result = new Object[(l + 1) / 2];
        for (int i = 0; i < l; i++) {
            if (i % 2 == 0) {
                result[count] = a[i];
                count++;
            }
        }
        return result;
    }
}
