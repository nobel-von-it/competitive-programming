package Kata;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Table {
    public static String multiTable(int n) {
        List<String> list = new ArrayList<>();
        for (int i = 1; i <= 10; i++) {
            list.add(i + " * " + n + " = " + (i * n));
        }
        return String.join("\n", list);
    }
    //or
    public static String multT(int n) {
        return IntStream.rangeClosed(1, 10)
                .mapToObj(i -> i + " * " + n + " = " + i*n)
                .collect(Collectors.joining("\n"));
    }
}
