package com.example.quality.controller;

import com.example.quality.dto.NewClient;
import com.example.quality.model.Client;
import com.example.quality.repository.ClientRepository;
import jakarta.validation.Valid;
import org.springframework.beans.BeanUtils;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;

@Controller
public class ClientController {
    private final ClientRepository repo;
    public ClientController(ClientRepository repo) { this.repo = repo; }

    @GetMapping("/")
    public String home(Model model) {
        model.addAttribute("clients", repo.findAllByOrderByIdDesc());
        return "index";
    }

    @GetMapping("/ajouter")
    public String showAddForm(Model model) {
        model.addAttribute("newClient", new NewClient());
        return "form";
    }

    @PostMapping("/ajouter")
    public String addClient(@Valid @ModelAttribute NewClient dto,
                             BindingResult errors) {
        if (errors.hasErrors()) return "form";
        Client c = new Client();
        BeanUtils.copyProperties(dto, c);
        repo.save(c);
        return "redirect:/";
    }
}
