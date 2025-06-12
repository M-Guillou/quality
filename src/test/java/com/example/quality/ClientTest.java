package com.example.quality;

import com.example.quality.model.Client;
import org.junit.jupiter.api.Test;

import java.time.LocalDate;
import java.util.Date;

import static org.junit.jupiter.api.Assertions.*;

public class ClientTest {

    @Test
    void testGettersAndSetters() {
        Client client = new Client();
        client.setNom("Doe");
        client.setPrenom("John");
        client.setAdresse("123 rue Exemple");
        client.setCodePostal("75000");
        client.setVille("Paris");
        LocalDate birthDate = LocalDate.of(1990, 1, 1);
        client.setDateNaissance(birthDate);

        assertEquals("Doe", client.getNom());
        assertEquals("John", client.getPrenom());
        assertEquals("123 rue Exemple", client.getAdresse());
        assertEquals("75000", client.getCodePostal());
        assertEquals("Paris", client.getVille());
        assertEquals(birthDate, client.getDateNaissance());
    }
}
