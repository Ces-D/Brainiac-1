pub const SUMMARIZATION_EXAMPLES: [&str; 3] = [
    "In 'The Innocent Man', John Grisham tells the shocking true story of four innocent men wrongfully convicted of murder in Ada, Oklahoma. Ron Williamson, a mentally-disturbed former baseball player, is wrongly accused of murdering Debbie Carter and was pursued with fabricated testimony and flawed science despite lacking hard evidence. Dennis Fritz, a regular guy, was also wrongly accused as an accomplice due to his friendship with Ron. The book highlights how easily justice can be miscarried, and how hard it is for innocent people to escape the system once they are trapped in it.",

    "Nothing in life is fixed or permanent, including ourselves, making it impossible to find lasting security. Suffering is not just physical pain   but also emotional and mental distress that is an inherent part of life. Buddhism rejects the idea of a permanent self or soul, instead seeing our identity as composed of temporary elements such as form, feeling-sensation, perception, etc. To overcome suffering, Buddhism begins with recognizing its nature and offering a path to liberation through understanding its cause - a deeply-rooted sense of 'I' - and following the Noble Eightfold Path. This eight-practice path involves cultivating moral restraint, mindfulness, and concentration to overcome suffering and achieve spiritual enlightenment, which can be achieved through regular practice of meditation that helps quiet the mind and observe thoughts, emotions, and impulses without judgment or attachment.",

    "The balance sheet provides an overview of a company's finances at a moment in time but lacks context for longer-term trends, making it necessary to compare with previous periods. To assess a company's financial well-being, investors can use various ratios derived from the balance sheet, such as the debt-to-equity ratio and acid-test ratio, which provide valuable insights alongside income statements and statement of cash flows. The balance sheet adheres to the accounting equation: Assets = Liabilities + Shareholders' Equity, reflecting that a company pays for its assets by either borrowing or investing equity. When a company takes on debt or issues equity, the balance sheet reflects these changes by increasing liabilities or shareholder equity, respectively, balancing the two sides of the equation."
];

pub const TITLE_EXAMPLES: [&str; 5] = [
    "The Girl With The Dragon Tattoo", 
    "Unwrapping the Intricate Interplay Between Energy Dependency and Macroeconomic Volatility in OECD Countries",
    "Evaluating the Role of GDP Per Capita, Air Pollution and Non-Economic Factors in Determining Health Expenditure: Evidence from Asian Region Using Instrumental Variables Techniques",
    "Your Table Is Ready: Tales of a New York City Maître D'",
    "The Silent Patient"
];

pub const KEYWORD_EXAMPLES: [&str; 5] = [
    "MachineLearning, ArtificialIntelligence, DataScience, PythonProgramming, DataAnalysis, Algorithms" ,
    "LovePoetry, NatureInspiration, FreeVerse, RhythmAndMeter, ImageryInPoetry, SocialCommentary",
    "ElectionCampaigns, GovernmentPolicy, SocialJustice, NationalSecurity, InternationalRelations, VotingRights",
    "MentalIllnessAwareness, AnxietyManagement, DepressionSupport, TraumaRecovery, MindfulnessPractices, MentalHealthStigma",
    "SingletonPattern, FactoryMethodPattern, ObserverPattern, StrategyPattern, DecoratorPattern, TemplateMethodPattern"
];

/// Selects examples from the given options based on the desired example count
/// Includes as many examples as possible
pub fn select_examples(options: Vec<&str>, desired_example_count: usize) -> Vec<String> {
    if desired_example_count > 0 && desired_example_count <= options.len() {
        options[0..desired_example_count]
            .iter()
            .copied()
            .map(|s| s.to_string())
            .collect()
    } else {
        options.iter().copied().map(|s| s.to_string()).collect()
    }
}
