use super::gen_common::*;
use super::gen_website::*;
use super::hash_store::*;
use super::pedia::*;
use super::sink::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, html, text};

fn gen_petalace(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    mut output: impl Write,
    mut _toc_sink: TocSink<'_>,
) -> Result<()> {
    let mut petalace: Vec<_> = pedia_ex.buff_cage.values().collect();
    petalace.sort_unstable_by_key(|p| (p.data.sort_index, p.data.id));
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Petalace - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Petalace"</h1></header>
                <div class="mh-table"><table>
                <thead><tr>
                    <th>"Name"</th>
                    <th>"Description"</th>
                    <th>"Health"</th>
                    <th>"Stamina"</th>
                    <th>"Attack"</th>
                    <th>"Defense"</th>
                </tr></thead>
                <tbody>
                { petalace.into_iter().map(|petalace| html!(<tr>
                    <td><div class="mh-icon-text">
                        {gen_rared_icon(petalace.data.rarity, "/resources/equip/030", [])}
                        <span>{gen_multi_lang(petalace.name)}</span>
                    </div></td>
                    <td><pre>{gen_multi_lang(petalace.explain)}</pre></td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[0], petalace.data.status_buff_limit[0])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[1], petalace.data.status_buff_limit[1])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[2], petalace.data.status_buff_limit[2])}</td>
                    <td>{text!("+{} / {}", petalace.data.status_buff_add_value[3], petalace.data.status_buff_limit[3])}</td>
                </tr>)) }
                </tbody>
                </table></div>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

fn gen_misc_page(
    hash_store: &HashStore,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    mut output: impl Write,
) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html lang="en">
            <head itemscope=true>
                <title>{text!("Misc - MHRice")}</title>
                { head_common(hash_store) }
            </head>
            <body>
                { navbar() }
                <main>
                <header><h1>"Miscellaneous"</h1></header>

                <a href="/misc/petalace.html">"Petalace"</a>
                </main>
                { right_aside() }
            </body>
        </html>
    );
    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_misc(
    hash_store: &HashStore,
    _pedia: &Pedia,
    pedia_ex: &PediaEx,
    config: &WebsiteConfig,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let folder = output.sub_sink("misc")?;
    let (path, toc_sink) = folder.create_html_with_toc("petalace.html", toc)?;
    gen_petalace(hash_store, pedia_ex, config, path, toc_sink)?;

    gen_misc_page(
        hash_store,
        pedia_ex,
        config,
        output.create_html("misc.html")?,
    )?;

    Ok(())
}
