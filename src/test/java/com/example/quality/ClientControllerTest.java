@SpringBootTest
@AutoConfigureMockMvc
public class ClientControllerTest {
    @Autowired private MockMvc mvc;
    @Test void testHome() throws Exception {
        mvc.perform(get("/"))
           .andExpect(status().isOk())
           .andExpect(view().name("index"));
    }
}
