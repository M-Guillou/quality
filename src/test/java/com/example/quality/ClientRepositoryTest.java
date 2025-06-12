package com.example.quality;

import com.example.quality.model.Client;
import com.example.quality.repository.ClientRepository;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.jdbc.AutoConfigureTestDatabase;
import org.springframework.boot.test.autoconfigure.orm.jpa.DataJpaTest;
import org.springframework.boot.test.context.SpringBootTest;

import java.time.LocalDate;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

@SpringBootTest
@AutoConfigureTestDatabase(replace = AutoConfigureTestDatabase.Replace.NONE)
public class ClientRepositoryTest {

    @Autowired
    private ClientRepository repo;

    @Test
    void findAllByOrderByIdDesc_ShouldReturnClientsInDescendingOrder() {
        Client c1 = new Client();
        c1.setNom("Alpha");
        c1.setPrenom("A");
        c1.setDateNaissance(LocalDate.of(1980, 1, 1));
        c1.setAdresse("123 Rue Alpha");
        c1.setCodePostal("75001"); 
        c1.setVille("Paris");
        repo.save(c1);

        Client c2 = new Client();
        c2.setNom("Bravo");
        c2.setPrenom("B");
        c2.setDateNaissance(LocalDate.of(1980, 1, 1));
        c2.setAdresse("456 Rue Bravo");
        c2.setCodePostal("75002");
        c2.setVille("Paris");
        repo.save(c2);

        List<Client> clients = repo.findAllByOrderByIdDesc();

        assertFalse(clients.isEmpty(), "List should not be empty");
        assertEquals(c2.getNom(), clients.get(0).getNom(), "First client should be last saved (descending order)");
        assertEquals(c1.getNom(), clients.get(1).getNom(), "Second client should be first saved");
    }
}
