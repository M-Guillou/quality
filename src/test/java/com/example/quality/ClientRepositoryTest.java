package com.example.quality;

import com.example.quality.model.Client;
import com.example.quality.repository.ClientRepository;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.orm.jpa.DataJpaTest;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

@DataJpaTest
public class ClientRepositoryTest {

    @Autowired
    private ClientRepository repo;

    @Test
    void findAllByOrderByIdDesc_ShouldReturnClientsInDescendingOrder() {
        Client c1 = new Client();
        c1.setNom("Alpha");
        c1.setPrenom("A");
        repo.save(c1);

        Client c2 = new Client();
        c2.setNom("Bravo");
        c2.setPrenom("B");
        repo.save(c2);

        List<Client> clients = repo.findAllByOrderByIdDesc();

        assertFalse(clients.isEmpty(), "List should not be empty");
        assertEquals(c2.getNom(), clients.get(0).getNom(), "First client should be last saved (descending order)");
        assertEquals(c1.getNom(), clients.get(1).getNom(), "Second client should be first saved");
    }
}
