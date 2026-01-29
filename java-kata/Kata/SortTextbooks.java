package Kata;

import java.util.List;

public class SortTextbooks {
    public static List<String > sort(List<String> textbooks) {
        textbooks.sort(String.CASE_INSENSITIVE_ORDER);
        return textbooks;
//        return textbooks.stream().sorted(String.CASE_INSENSITIVE_ORDER).toList();
    }
}
