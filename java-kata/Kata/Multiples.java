package Kata;


import java.util.ArrayList;
import java.util.List;

public class Multiples{
    public static int[] find(int s, int e) {
        List<Integer> list = new ArrayList<>();
        for (int i = s; i <= e; i += s) {
            if (i % s == 0)
                list.add(i);
        }
//        int[] result = new int[list.size()];
//        for (int i = 0; i < list.size(); i++) {
//            result[i] = list.get(i);
//        }
//        return result;
        return list.stream().mapToInt(Integer::intValue).toArray();
    }
}
