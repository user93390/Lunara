import { useEffect, useState } from "react";
import "../styles/plugin.scss";

function PluginBoard() {
    const [plugins, setPlugins] = useState<Plugin[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [page, setPage] = useState(1);

    useEffect(() => {
        setLoading(true);
        get_plugins(page)
            .then((data) => {
                setPlugins(data);
                setLoading(false);
            })
            .catch((err) => {
                setError(err.message);
                setLoading(false);
            });
    }, [page]);

    if (loading) return <div className="plugin-status">Loading...</div>;
    if (error) return <div className="plugin-status">Error: {error}</div>;
    if (plugins.length === 0) return <div className="plugin-status">No plugins found</div>;

    return (
        <div className="plugin-container">
            <div className="plugin-list">
                {plugins.map((plugin) => (
                    <a 
                        className="plugin-row" 
                        key={plugin.slug}
                        href={`https://hangar.papermc.io/${plugin.owner}/${plugin.slug}`}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <img className="plugin-icon" src={plugin.icon} alt={plugin.name} />
                        <div className="plugin-info">
                            <span className="plugin-name">{plugin.name}</span>
                            <span className="plugin-owner">{plugin.owner}</span>
                        </div>
                        <div className="plugin-stats">
                            <span className="stat">
                                <svg viewBox="0 0 24 24"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>
                                {plugin.stars.toLocaleString()}
                            </span>
                            <span className="stat">
                                <svg viewBox="0 0 24 24"><path d="M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z"/></svg>
                                {plugin.downloads.toLocaleString()}
                            </span>
                        </div>
                    </a>
                ))}
            </div>
            <div className="plugin-pagination">
                <button onClick={() => setPage(p => Math.max(1, p - 1))} disabled={page === 1}>Previous</button>
                <span>{page}</span>
                <button onClick={() => setPage(p => p + 1)} disabled={plugins.length === 0}>Next</button>
            </div>
        </div>
    );
}

async function get_plugins(page_number: number): Promise<Plugin[]> {
    const response: Response = await fetch("http://localhost:5000/mc/plugin/trending?trending=" + page_number);

    if (!response.ok) {
        return [];
    }

    const jsonData: TrendingPlugin[] = await response.json();

    return jsonData.map((plugin) => ({
        icon: plugin.avatarUrl,
        name: plugin.name,
        downloads: plugin.stats.downloads,
        stars: plugin.stats.stars,
        owner: plugin.namespace.owner,
        slug: plugin.namespace.slug,
        download_url: plugin.download_url,
    }));
}

interface TrendingPlugin {
    name: string;
    namespace: {
        owner: string;
        slug: string;
    };
    stats: {
        downloads: number;
        stars: number;
    };
    avatarUrl: string;
    download_url: string;
}

interface Plugin {
    icon: string;
    name: string;
    downloads: number;
    stars: number;
    owner: string;
    slug: string;
    download_url: string;
}

export default PluginBoard