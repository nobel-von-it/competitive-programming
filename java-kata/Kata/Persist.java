package Kata;

import java.util.Arrays;
import java.util.stream.IntStream;

public class Persist {
    public static int persistence(long n) {
        int res = 0;
        while (n >= 10) {
            n = Arrays.stream(String.valueOf(n).split("")).mapToInt(Integer::parseInt).reduce((a, e) -> a*e).getAsInt();
            System.out.println(n);
            res++;
        }
        return res;
    }
    public static int persist(long n) {
        long m = 1, r = n;
        if (r / 10 == 0) return 0;
        for (r = n; r != 0; r/=10)
            m *= r % 10;
        return persist(m)+1;
    }
}
