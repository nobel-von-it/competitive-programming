package Kata;

public class DayOfWeek {
    public static String getDay(int n) {
        String[] days = new String[]{"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"};
        if (n > 0 && n <= days.length) {
            return days[n-1];
        }
        return "Wrong, please enter a number between 1 and 7";
    }
}
