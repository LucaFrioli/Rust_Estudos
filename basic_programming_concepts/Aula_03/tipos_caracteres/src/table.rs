macro_rules! separator {
    ($width:expr) => {
        println!("|{:-<1$}|", "", $width);
    };
}

macro_rules! resumo_tabela {
    () => {
        println!("\n\n{:=<50}", "");
        println!("|{:^48}|", "Resumo sobre a Tabela de Caracteres e Rust");
        println!("{:=<50}\n", "");
        println!("1. Tabela Representa:");
        println!("   - Uma lista de caracteres com suas descrições e códigos Unicode.");
        println!("   - Inclui emojis e símbolos especiais, como o coração vermelho.");
        println!();
        println!("2. Nuances do Tipo Escalar 'char':");
        println!("   - Em Rust, 'char' representa um ponto de código Unicode (U+0000 a U+10FFFF).");
        println!("   - Alguns caracteres visuais, como emojis, são compostos por múltiplos 'char'.");
        println!();
        println!("3. Unicode e Emojis:");
        println!("   - Emojis podem incluir modificadores, como U+FE0F, que definem variações de exibição.");
        println!("   - Exemplo: O coração vermelho (❤️ ) é composto por dois pontos de código:");
        println!("     - U+2764: Coração básico.");
        println!("     - U+FE0F: Modificador que força a renderização como emoji.");
        println!();
        println!("4. Boas Práticas:");
        println!("   - Sempre considere clusters de grafemas ('graphemes') para lidar com caracteres visuais.");
        println!("{:=<50}\n\n", "");
    };
}

pub fn display_table() {
    const TABLE_WIDTH: usize = 160;
    separator!(TABLE_WIDTH+7);
    println!("| {:^width$} | {:^width$} | {:^width$} |", "Caractere", "Unicode", "Descrição", width = TABLE_WIDTH / 3);
    separator!(TABLE_WIDTH+7);

    let caracteres: [(&str, &str, &str); 88] = [
        ("A", "U+0041", "Letra maiúscula A"),
        ("Ã", "U+00C3", "Letra maiúscula A com til"),
        ("Á", "U+00C1", "Letra maiúscula A com acento agudo"),
        ("À", "U+00C0", "Letra maiúscula A com til"),
        ("a", "U+0061", "Letra minúscula a"),
        ("ã", "U+00E3", "Letra minúscula a com til"),
        ("á", "U+00E1", "Letra minúscula a com acento agudo"),
        ("à", "U+00E0", "Letra minúscula a com til"),
        ("B", "U+0042", "Letra maiúscula B"),
        ("b", "U+0062", "Letra minúscula b"),
        ("C", "U+0043", "Letra maiúscula C"),
        ("c", "U+0063", "Letra minúscula c"),
        ("Ç", "U+00C7", "Letra maiúscula C cedilha"),
        ("ç", "U+00E7", "Letra minúscula c cedilha"),
        ("D", "U+0044", "Letra maiúscula D"),
        ("d", "U+0064", "Letra minúscula d"),
        ("E", "U+0045", "Letra maiúscula E"),
        ("e", "U+0065", "Letra minúscula e"),
        ("É", "U+00C9", "Letra maiúscula E com acento agudo"),
        ("é", "U+00E9", "Letra minúscula e com acento agudo"),
        ("Ê", "U+00CA", "Letra maiúscula E com circunflexo"),
        ("ê", "U+00EA", "Letra minúscula e com circunflexo"),
        ("F", "U+0046", "Letra maiúscula F"),
        ("f", "U+0066", "Letra minúscula f"),
        ("G", "U+0047", "Letra maiúscula G"),
        ("g", "U+0067", "Letra minúscula g"),
        ("H", "U+0048", "Letra maiúscula H"),
        ("h", "U+0068", "Letra minúscula h"),
        ("I", "U+0049", "Letra maiúscula I"),
        ("i", "U+0069", "Letra minúscula i"),
        ("Í", "U+00CD", "Letra maiúscula I com acento agudo"),
        ("í", "U+00ED", "Letra minúscula i com acento agudo"),
        ("J", "U+004A", "Letra maiúscula J"),
        ("j", "U+006A", "Letra minúscula j"),
        ("K", "U+004B", "Letra maiúscula K"),
        ("k", "U+006B", "Letra minúscula k"),
        ("L", "U+004C", "Letra maiúscula L"),
        ("l", "U+006C", "Letra minúscula l"),
        ("M", "U+004D", "Letra maiúscula M"),
        ("m", "U+006D", "Letra minúscula m"),
        ("N", "U+004E", "Letra maiúscula N"),
        ("n", "U+006E", "Letra minúscula n"),
        ("O", "U+004F", "Letra maiúscula O"),
        ("o", "U+006F", "Letra minúscula o"),
        ("Ó", "U+00D3", "Letra maiúscula O com acento agudo"),
        ("ó", "U+00F3", "Letra minúscula o com acento agudo"),
        ("Ô", "U+00D4", "Letra maiúscula O com circunflexo"),
        ("ô", "U+00F4", "Letra minúscula o com circunflexo"),
        ("Õ", "U+00D5", "Letra maiúscula O com til"),
        ("õ", "U+00F5", "Letra minúscula o com til"),
        ("P", "U+0050", "Letra maiúscula P"),
        ("p", "U+0070", "Letra minúscula p"),
        ("Q", "U+0051", "Letra maiúscula Q"),
        ("q", "U+0071", "Letra minúscula q"),
        ("R", "U+0052", "Letra maiúscula R"),
        ("r", "U+0072", "Letra minúscula r"),
        ("S", "U+0053", "Letra maiúscula S"),
        ("s", "U+0073", "Letra minúscula s"),
        ("T", "U+0054", "Letra maiúscula T"),
        ("t", "U+0074", "Letra minúscula t"),
        ("U", "U+0055", "Letra maiúscula U"),
        ("u", "U+0075", "Letra minúscula u"),
        ("Ú", "U+00DA", "Letra maiúscula U com acento agudo"),
        ("ú", "U+00FA", "Letra minúscula u com acento agudo"),
        ("V", "U+0056", "Letra maiúscula V"),
        ("v", "U+0076", "Letra minúscula v"),
        ("W", "U+0057", "Letra maiúscula W"),
        ("w", "U+0077", "Letra minúscula w"),
        ("X", "U+0058", "Letra maiúscula X"),
        ("x", "U+0078", "Letra minúscula x"),
        ("Y", "U+0059", "Letra maiúscula Y"),
        ("y", "U+0079", "Letra minúscula y"),
        ("Z", "U+005A", "Letra maiúscula Z"),
        ("z", "U+007A", "Letra minúscula z"),
        ("\\n", "U+000A", "Nova linha (line feed)"),
        ("\\r", "U+000D", "Retorno de carro (carriage return)"),
        ("\\t", "U+0009", "Tabulação horizontal"),
        ("\\u{20}", "U+0020", "Espaço"),
        ("\\u{A0}", "U+00A0", "Espaço sem quebra (non-breaking space)"),
        ("\\u{200B}", "U+200B", "Espaço invisível (zero-width space)"),
        ("😊", "U+1F60A", "Emoji: face sorrindo"),
        ("😁", "U+1F601", "Emoji: face sorrindo com olhos sorridentes"),
        ("😭", "U+1F62D", "Emoji: face chorando"),
        ("😍", "U+1F60D", "Emoji: rosto apaixonado"),
        ("👍", "U+1F44D", "Emoji: polegar para cima"),
        ("❤️", "U+2764", "Emoji: coração vermelho"),
        ("🎉", "U+1F389", "Emoji: festa"),
        ("🚀", "U+1F680", "Emoji: foguete"),
    ];
    
    

    for (caractere, unicode, descricao) in &caracteres {
        let temp: &&str = caractere;

        if temp.is_ascii() {
            println!("| {:^width$} | {:^width$} | {:<width$} |", caractere, unicode, descricao, width = TABLE_WIDTH / 3);
            continue;
        }

        if *descricao == "Emoji: coração vermelho"{
            println!("|  {:^width$} | {:^width$} | {:<width$} |", caractere, unicode, descricao, width = TABLE_WIDTH / 3);
            continue;
        }

        let temp: Vec<_> = temp.chars().collect();

        for i in temp{
            let i: u32 = i as u32;
            if i < 10084{
                println!("| {:^width$} | {:^width$} | {:<width$} |", caractere, unicode, descricao,width = TABLE_WIDTH / 3);
            }

            if i > 10084{
                println!("| {:^width$}| {:^width$} | {:<width$} |", caractere, unicode, descricao, width = TABLE_WIDTH / 3);
            }
        }        

    }

    separator!(TABLE_WIDTH+7);

    resumo_tabela!();


}
