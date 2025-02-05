use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationResponse},
    Ollama,
};

use super::response::JSONResponse;
use crate::model::{ArticleGenre, ResponseOutputType};

pub struct OutputFormatter<'a> {
    instance: &'a Ollama,
    model: String,
}

impl<'a> OutputFormatter<'a> {
    pub fn new(instance: &'a Ollama, model: String) -> Self {
        Self { instance, model }
    }

    pub async fn format_output(
        &self,
        content: String,
        output_type: ResponseOutputType,
    ) -> ollama_rs::error::Result<GenerationResponse> {
        let system = "You are a REST API and can only respond in JSON. You have been given a strict structure to respond in.";
        let example_responses = match output_type {
            ResponseOutputType::Description => [
                JSONResponse::from(" The Middle Way in Buddhism is a philosophy that emphasizes balance, harmony, and respect for the intrinsic dignity of all life. Rooted in the teachings of Shakyamuni Buddha, it advocates against extreme views, whether they be luxurious indulgence or harsh asceticism, promoting instead a moderate path that fosters inner peace and wisdom. Zhiyi further developed this concept by introducing three truths: the truth of temporary existence (physical aspects), the truth of non-substantiality (mental and spiritual aspects), and the essence of life that unifies these opposites. This holistic view underscores the interconnectedness of body, mind, and environment. Nichiren expanded on this by describing life as an elusive reality, neither merely existing nor non-existing but embodying both qualities. He stressed the inherent dignity in all life, encouraging a compassionate and harmonious approach to existence. This philosophy aligns with Gandhi's assertion about considering the poorest man, linking the Middle Way to social justice. The Soka Gakkai envisions the Middle Way as a path that transforms individuals and societies toward happiness and coexistence. It involves living positively, constantly reflecting on actions to ensure they align with humanity's ethical path. This principle is seen as a guiding light against the extremes of modernity, offering a pathway towards peace and fulfillment. In essence, the Middle Way is not just about personal enlightenment but also about fostering social harmony and collective well-being, respecting life's sanctity, and seeking wisdom to benefit both individuals and society.") .to_string(),
                JSONResponse::from( " The article explores prevalent competitive Pokémon strategies, highlighting their effectiveness and complexity. Despite the vast array of Pokémon and moves, certain strategies consistently dominate, offering both offensive and defensive advantages. 1. **Perish Traps**: Utilize the Perish Song move to KO opponents after three turns. Enhanced with abilities like Shadow Tag, these teams trap foes, making it easier to apply the finishing move. 2. **Hazard Stacks**: Employ multiple Hazard moves such as Stealth Rock and Spikes to progressively chip away at opponents' health. Teams often use dedicated setters and a mix of defensive and offensive Pokémon to maintain balance. 3. **Sun Teams**: Leverage harsh sunlight to boost fire-type attacks and abilities like Chlorophyll. These teams are known for their offensive prowess, especially with Dought-boosted Pokémon. 4. **Rain Teams**: Utilize the Rain Weather Effect to enhance Swift Swim moves and reduce fire damage. This strategy allows for swift sweeps and defensive positioning with rain-resistant Pokémon. 5. **Sandstorm Teams**: Initiate sandstorms with Tyranitar's Sandstream ability, causing widespread damage unless countered by specific types like rock or ground. These strategies illustrate how strategic team compositions and weather conditions can tilt the balance in competitive Pokémon battles, making them both challenging and thrilling for players. ")
                .to_string(),
                JSONResponse::from( " Nietzsche's exploration of nihilism serves as a cornerstone in understanding the intellectual landscape of the late 19th and early 20th centuries. Nihilism, as defined by Nietzsche, posits the absence of inherent meaning or value in life, challenging conventional moral and religious frameworks. He viewed this stance with apprehension, recognizing its potential to undermine societal values and belief systems. In the 20th century, nihilistic themes became prominent in philosophy and literature, particularly through the works of existentialists like Sartre and Camus, who grappled with the 'absurd' nature of existence. While these thinkers often expressed despair, they also sought ways to live authentically despite the lack of inherent purpose. This era witnessed a blend of gloom and resilience as individuals navigated the complexities of meaninglessness. Postmodernity shares a skepticism towards grand narratives and meta-discourses, aligning with nihilistic ideas about the baselessness of values. However, postmodernity doesn't equate to nihilism itself; it's more about the critical engagement with these stories rather than their outright rejection. Nietzsche believed that if we deconstruct existing interpretations, we might uncover new paths forward—a process akin to postmodern deconstruction leading to reconstruction. Language plays a crucial role in this discourse, as Nietzsche argued that language and metaphors shape our understanding of reality. If language fails us, so does our ability to communicate meaning, underscoring the epistemological challenges inherent in nihilism. Social critics and artists have often depicted themes of alienation and existential despair, reflecting nihilistic ideas about the absence of inherent purpose. While Nietzsche saw nihilism as a necessary destruction leading to potential renewal, others may view it as an insurmountable problem without transcendence or belief. Practically, nihilism raises questions about moral relativism and how individuals navigate their lives without inherent meaning. Some counter this by finding localized meaning in relationships, art, and personal goals, suggesting that meaning can be constructed on a smaller scale. Antifoundationalism, the notion that there's no foundation for knowledge or value, ties into postmodern views of truth as constructed rather than discovered. This perspective resonates with nihilistic themes about the baselessness of values. In conclusion, Nietzsche's ideas on nihilism provide a rich tapestry to explore its evolution and impact across centuries. From its philosophical origins to its influence in shaping artistic and cultural responses, nihilism continues to be a significant lens through which we view the human condition. It challenges us to consider the possibilities of meaning-making in an era marked by doubt and inquiry. ")
                .to_string(),
            ],
            ResponseOutputType::Title => [
                JSONResponse::from("The Fall of the Roman Empire").to_string(),
                JSONResponse::from("The Economic Impact of Climate Change").to_string(),
                JSONResponse::from("The Relationship Between Art and Politics").to_string(),
            ],
            ResponseOutputType::Genre => [
                JSONResponse::from(ArticleGenre::Art.to_string()).to_string(),
                JSONResponse::from(ArticleGenre::Opinion.to_string()).to_string(),
                JSONResponse::from(ArticleGenre::Technology.to_string()).to_string(),
            ],
            ResponseOutputType::Keywords => [
                JSONResponse::from(vec!["Roman Empire".to_string(), "History".to_string()])
                    .to_string(),
                JSONResponse::from(vec!["Climate Change".to_string(), "Economy".to_string()])
                    .to_string(),
                JSONResponse::from(vec!["Art".to_string(), "Politics".to_string()]).to_string(),
            ],
        };

        let request = GenerationRequest::new(
            self.model.clone(),
            format!(
                "Here are examples of appropriate responses: {}\nHere is the data that must be formatted: {}",
                format_conversation(example_responses),
                content
            ),
        )
        .system(system.to_string())
        .format(ollama_rs::generation::parameters::FormatType::Json);

        self.instance.generate(request).await
    }
}

fn format_conversation(responses: [String; 3]) -> String {
    let mut conversation = String::new();
    for response in responses.iter() {
        conversation.push_str(&format!("{}\n", response));
    }
    conversation
}
