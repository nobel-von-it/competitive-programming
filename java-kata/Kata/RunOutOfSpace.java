package Kata;

public class RunOutOfSpace {
    public static String[] spacey(String[] array) {
        for (int i = 0; i < array.length - 1; i++) {
            array[i+1] = array[i] + array[i+1];
        }
        return array;
    }
}
