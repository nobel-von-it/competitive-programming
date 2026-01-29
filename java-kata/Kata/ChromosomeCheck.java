package Kata;

public class ChromosomeCheck {
    public static String chromosomeCheck(String s) {
        return s.contains("Y") ?
                "Congratulations! You're going to have a son." :
                "Congratulations! You're going to have a daughter.";
    }
}
