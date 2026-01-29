package Kata;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Diff {
    public static int[] arrayDiff(int[] a, int[] b) {
        List<Integer> list = new ArrayList<>();
        Set<Integer> set = new HashSet<>();
        if (b.length == 0 || a.length == 0)
            return a;
        for (int k : b) {
            int count = 0;
            for (int j : a)
                if (k == j)
                    count++;
            if (count > 0)
                set.add(k);
        }
        System.out.println(set);
        for (int i : a)
            if (!set.contains(i))
                list.add(i);

        int[] res = new int[list.size()];
        for (int i = 0; i < list.size(); i++)
            res[i] = list.get(i);
        return res;
    }
    public static int[] arrDiff(int[] a, int[] b) {
        List<Integer> A = Arrays.stream(a).boxed().toList();
        List<Integer> B = Arrays.stream(b).boxed().toList();
        A.removeAll(B);
        return A.stream().mapToInt(Integer::intValue).toArray();
    }
    public static int[] arrDifff(int[] a, int[] b) {
        return IntStream.of(a).filter(x -> IntStream.of(b).noneMatch(y -> y == x)).toArray();
    }
}
