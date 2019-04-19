package se.hof.adventofcode.eighteen;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class DayEightTest {

    DayEight.Node node;

    @Test
    public void parseChildlessNodeTest() {
        String s = "0 1 99";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(99, n.sum());

    }

}
