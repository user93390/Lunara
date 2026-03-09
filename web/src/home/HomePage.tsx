/*
 Copyright 2026 seasnail1

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.

 */



import {LineChart, type LineChartProps} from "@mui/x-charts";
import './styles/home.scss'
import {ThemeToggle} from "../components/theme/ThemeToggle.tsx";

import {
    FormControl,
    FormControlLabel, FormGroup,
    InputLabel,
    MenuItem,
    Paper,
    Select, Slider,
    Switch,
    Typography,
    useTheme
} from "@mui/material";
import {useCallback, useEffect, useState} from "react";

interface InstalledPlugin {
    name: string;
    version: string;
}

function HomePage() {
    const prof: number[] = [
        10,
        11,
        13,
        15,
        12,
        11,
        9,
        13,
        15,
        10,
        9
    ];

    const theme = useTheme();
    const primaryColor = theme.palette.primary.main;
    const chart = create_chart_config(prof, "%", primaryColor);

    const [installedPlugins, setInstalledPlugins] = useState<InstalledPlugin[]>([]);
    const [pluginsLoading, setPluginsLoading] = useState(true);

    const fetchInstalledPlugins = useCallback(async () => {
        setPluginsLoading(true);

        try {
            const serverRes = await fetch("http://localhost:5000/mc/server/list");
            if (!serverRes.ok) return;

            const servers = await serverRes.json();
            const parsed = typeof servers === "string" ? JSON.parse(servers) : servers;
            if (parsed.length === 0) return;

            const serverName = parsed[0].name ?? "unknown";
            const pluginRes = await fetch(`http://localhost:5000/mc/server/${serverName}/plugin/list`);
            if (!pluginRes.ok) return;

            const plugins: InstalledPlugin[] | null = await pluginRes.json();
            setInstalledPlugins(plugins ?? []);
        } catch {
            setInstalledPlugins([]);
        } finally {
            setPluginsLoading(false);
        }
    }, []);

    useEffect(() => {
        fetchInstalledPlugins();
    }, [fetchInstalledPlugins]);

    return (
        <>
            <div className="main">
                <header className="page-header">
                    <Typography variant="h4" component="h1" fontWeight={600}>
                        {welcome_state() + ", " + get_user()}
                    </Typography>
                    <ThemeToggle/>
                </header>

                <div className="grid-parent">
                    <Paper className="grid-child" elevation={2}>
                        <div className="chart-toolbar">
                            <Typography variant="h6" fontWeight={500}>
                                Analytics
                            </Typography>
                            <FormControl size="small" sx={{minWidth: 160}}>
                                <InputLabel>Chart type</InputLabel>
                                <Select label="Chart type" defaultValue="players">
                                    <MenuItem value="ram">Ram Usage</MenuItem>
                                    <MenuItem value="cpu">Cpu Usage</MenuItem>
                                    <MenuItem value="players">Player Count</MenuItem>
                                </Select>
                            </FormControl>
                        </div>
                        <div className="chart-container">
                            <LineChart
                                {...chart}
                                sx={{
                                    '& .MuiAreaElement-root': {
                                        fill: 'url(#area-gradient)',
                                    },
                                }}
                            >
                                <defs>
                                    <linearGradient id="area-gradient" x1="0" y1="0" x2="0" y2="1">
                                        <stop offset="0%" stopColor={primaryColor} stopOpacity={0.4} />
                                        <stop offset="100%" stopColor={primaryColor} stopOpacity={0} />
                                    </linearGradient>
                                </defs>
                            </LineChart>
                        </div>
                    </Paper>

                    <Paper className="grid-child" elevation={2}>
                        <Typography variant="h6" fontWeight={500}>
                            Players Online

                        </Typography>
                    </Paper>

                    <Paper className="grid-child" elevation={2}>
                        <Typography variant="h6" fontWeight={500}>
                            Installed Plugins
                        </Typography>
                    </Paper>

                    <Paper className="grid-child" elevation={2}>
                        <Typography variant="h6" fontWeight={500}>
                            Quick Options

                            <FormGroup>
                                <FormControlLabel control={<Switch defaultChecked />} label="Enable Whitelist"/>
                                <div>
                                    <p>Max Players</p>
                                    <Slider defaultValue={50} aria-label="Default" valueLabelDisplay="auto" max={50}/>
                                </div>
                            </FormGroup>
                        </Typography>
                    </Paper>
                </div>
            </div>
        </>
    )
}

function create_chart_config(data: number[], suffix: string, color: string): LineChartProps {
    return {
        xAxis: [
            {
                data: data.map((_, i) => i + 1),
                scaleType: 'point',
                tickLabelStyle: {display: 'none'},
                tickSize: 0,
                label: '',
            }
        ],
        yAxis: [
            {
                id: 'profit-axis',
                position: 'left',
                valueFormatter: (val: string) => val + suffix,
                max: 100
            },
        ],
        series: [
            {
                data: data,
                label: 'Player Count',
                yAxisId: 'profit-axis',
                color,
                showMark: false,
                area: true,
            },
        ],
        height: 350,
    }
}

function get_user() {
    // replace with cookie logic.
    return "seasnail1"
}

function welcome_state(): string {
    const hour: number = new Date().getHours();

    if (hour >= 5 && hour < 12) return "Good morning"
    if (hour >= 12 && hour < 17) return "Good afternoon"
    if (hour >= 17 && hour < 21) return "Good evening"
    return "night";
}

export default HomePage;