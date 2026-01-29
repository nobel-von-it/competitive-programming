package Kata;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Outlier {
    public static int find(int[] a) {
        List<Integer> odd = new ArrayList<>();
        List<Integer> even = new ArrayList<>();
        for (int i: a) {
            if (i % 2 == 0)
                even.add(i);
            else
                odd.add(i);
        }
        return odd.size() == 1 ? odd.get(0) : even.get(0);
    }
    public static int findd(int[] a) {
        int sum = Arrays.stream(a).limit(3).map(i -> Math.abs(i) % 2).sum();
        int mod = (sum == 0 || sum == 1) ? 1 : 0;

        return Arrays.stream(a).parallel().filter(i -> Math.abs(i) % 2 == mod).findFirst().getAsInt();
    }
}
