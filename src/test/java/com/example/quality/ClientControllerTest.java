package com.example.quality;

import com.example.quality.model.Client;
import com.example.quality.repository.ClientRepository;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.web.servlet.MockMvc;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.verify;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.*;

@SpringBootTest
@AutoConfigureMockMvc
public class ClientControllerTest {

    @Autowired private MockMvc mvc;

    @MockBean private ClientRepository repo;

    @Test void testHome() throws Exception {
        mvc.perform(get("/"))
                .andExpect(status().isOk())
                .andExpect(view().name("index"));
    }

    @Test void testShowAddForm() throws Exception {
        mvc.perform(get("/ajouter"))
                .andExpect(status().isOk())
                .andExpect(view().name("form"))
                .andExpect(model().attributeExists("newClient"));
    }

    @Test void testAddClientSuccess() throws Exception {
        mvc.perform(post("/ajouter")
                        .param("nom", "Doe")
                        .param("prenom", "John")
                        .param("dateNaissance", "1990-01-01")
                        .param("adresse", "123 rue Exemple")
                        .param("cp", "75000")
                        .param("ville", "Paris"))
                .andExpect(status().is3xxRedirection())
                .andExpect(redirectedUrl("/"));

        verify(repo).save(any(Client.class));
    }

    @Test void testAddClientValidationError() throws Exception {
        mvc.perform(post("/ajouter")
                        .param("nom", "")  // Invalid: required field is empty
                        .param("prenom", "John")
                        .param("dateNaissance", "1990-01-01")
                        .param("adresse", "123 rue Exemple")
                        .param("cp", "75000")
                        .param("ville", "Paris"))
                .andExpect(status().isOk())
                .andExpect(view().name("form"));
    }
}
