package Kata;

import java.util.LinkedList;

public class RemoveDupl {
    public static int[] solve(int[] a) {
        var list = new LinkedList<Integer>();
        for (int i = a.length - 1; i >= 0; i--) {
            if (!list.contains(a[i]))
                list.addFirst(a[i]);
        }
        return list.stream().mapToInt(i -> i).toArray();
    }
}
