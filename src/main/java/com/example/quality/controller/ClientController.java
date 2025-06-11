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
