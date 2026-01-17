/*
 * Copyright 2025 seasnail1
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import 'package:flutter/material.dart';
import 'package:flutter_feather_icons/flutter_feather_icons.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:fl_chart/fl_chart.dart';
import 'colours.dart';

import 'package:logger/logger.dart';
import 'ui_config.dart';

final ValueNotifier<ThemeMode> themeObserver = ValueNotifier(ThemeMode.system);

final String githubLink = 'https://github.com/user93390/lunara/';
final String username = 'user93390';
final String version = '1.0.0';

void main() {
  var logger = Logger(printer: PrettyPrinter());

  logger.i("Starting Lunara v$version...");

  runApp(const LunaraApp());
}

class LunaraApp extends StatelessWidget {
  const LunaraApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<ThemeMode>(
      valueListenable: themeObserver,
      builder: (context, themeMode, _) {
        return MaterialApp(
          themeMode: themeMode,
          theme: ThemeData(
            useMaterial3: true,
            colorScheme: ColorScheme.fromSeed(
              seedColor: Colours.seedColourLight,
              brightness: Brightness.light,
            ),
            textTheme: TextTheme(
              titleLarge: GoogleFonts.oswald(
                fontSize: 30,
                fontStyle: FontStyle.italic,
              ),
              bodyMedium: GoogleFonts.merriweather(),
              displaySmall: GoogleFonts.pacifico(),
            ),
          ),
          darkTheme: ThemeData(
            useMaterial3: true,
            colorScheme: ColorScheme.fromSeed(
              seedColor: Colours.seedColourDark,
              brightness: Brightness.dark,
            ),
            textTheme: TextTheme(
              titleLarge: GoogleFonts.oswald(
                fontSize: 30,
                fontStyle: FontStyle.italic,
              ),
              bodyMedium: GoogleFonts.merriweather(),
              displaySmall: GoogleFonts.pacifico(),
            ),
          ),
          home: Scaffold(
            appBar: AppBar(title: const Toolbar()),
            // add a bold centered text "Lunara Dashboard" above the main body
            body: Column(
              children: [
                const SizedBox(height: 20),
                FancyText(
                  text: 'Lunara Dashboard',
                  fontWeight: FontWeight.bold,
                ),
                const SizedBox(height: 20),
                Expanded(
                  child: SingleChildScrollView(
                    child: Center(child: MainBody()),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class MainBody extends StatefulWidget {
  const MainBody({super.key});

  @override
  State<MainBody> createState() => _MainBodyState();
}

class _MainBodyState extends State<MainBody> {
  final List<String> _lines = [];

  void addLine(String line) {
    setState(() => _lines.add(line));
  }

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final backgroundColor = isDark
        ? Colours.bodyColorDark
        : Colours.bodyColorLight;
    final borderColor = isDark ? Colours.borderDark : Colours.borderLight;

    return Container(
      alignment: Alignment.center,
      child: Container(
        alignment: Alignment.topLeft,
        clipBehavior: Clip.antiAlias,
        decoration: BoxDecoration(
          color: backgroundColor,
          borderRadius: BorderRadius.circular(UIConfig.mainBodyRadius),
          border: Border.all(
            color: borderColor,
            width: UIConfig.borderThickness,
          ),
        ),
        width: UIConfig.mainBodyWidth,
        height: UIConfig.mainBodyHeight,
        child: Stack(
          fit: StackFit.expand,
          children: [
            Padding(
              padding: const EdgeInsets.all(UIConfig.mainPadding),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Graph(
                        width: UIConfig.graphWidth,
                        height: UIConfig.graphHeight,
                        title: FancyText(
                          text: 'Player Insights',
                          icon: Icon(FeatherIcons.barChart2),
                          fontWeight: FontWeight.normal,
                        ),
                      ),
                      SizedBox(height: UIConfig.gapLarge),
                      Graph(
                        width: UIConfig.graphWidth,
                        height: UIConfig.graphHeight,
                        title: FancyText(
                          text: 'RAM Usage',
                          icon: Icon(FeatherIcons.barChart2),
                          fontWeight: FontWeight.normal,
                        ),
                      ),
                      SizedBox(height: UIConfig.gapLarge),
                      TerminalInput(onSubmitted: addLine),
                    ],
                  ),
                  const SizedBox(width: UIConfig.gapLarge),
                  SizedBox(
                    width: UIConfig.terminalOutputWidth,
                    height: UIConfig.terminalOutputHeight,
                    child: Padding(
                      padding: const EdgeInsets.only(
                        top: UIConfig.terminalOutputTopPadding,
                      ),
                      child: TerminalOutput(lines: _lines),
                    ),
                  ),
                ],
              ),
            ),
            Positioned(
              top: 16,
              right: 16,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  ElevatedButton(
                    onPressed: () async => {},
                    child: FancyText(
                      text: 'Lunara $version',
                      icon: Icon(FeatherIcons.info),
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class FancyText extends StatelessWidget {
  const FancyText({
    super.key,
    required this.text,
    required this.fontWeight,
    this.icon,
    this.size = 16,
  });

  final String text;
  final Icon? icon;
  final double size;

  final FontWeight fontWeight;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final textColor = isDark ? Colours.foregroundDark : Colours.foregroundLight;

    return Row(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        if (icon != null) ...[icon!, const SizedBox(width: 8)],
        Text(
          text,
          style: TextStyle(
            fontWeight: fontWeight,
            color: textColor,
            fontSize: size,
          ),
          textAlign: TextAlign.center,
        ),
      ],
    );
  }
}

class TerminalInput extends StatelessWidget {
  const TerminalInput({super.key, required this.onSubmitted});

  final ValueChanged<String> onSubmitted;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final textColor = isDark ? Colours.foregroundDark : Colours.foregroundLight;
    final fillColor = isDark ? Colours.backgroundDark : Colours.backgroundLight;
    final borderColor = isDark ? Colours.borderDark : Colours.borderLight;

    return SizedBox(
      width: UIConfig.terminalInputWidth,
      child: TextField(
        style: TextStyle(color: textColor, fontFamily: 'Courier New'),
        decoration: InputDecoration(
          filled: true,
          fillColor: fillColor,
          border: OutlineInputBorder(
            borderSide: BorderSide(
              color: borderColor,
              width: UIConfig.borderThickness,
            ),
            borderRadius: BorderRadius.circular(UIConfig.cornerRadius),
          ),
          hintText: 'Enter command...',
          hintStyle: TextStyle(color: textColor, fontFamily: 'Courier New'),
        ),
        onSubmitted: onSubmitted,
      ),
    );
  }
}

class TerminalOutput extends StatelessWidget {
  const TerminalOutput({super.key, required this.lines});

  final List<String> lines;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final textColor = isDark ? Colours.foregroundDark : Colours.foregroundLight;
    final bgColor = isDark ? Colours.backgroundDark : Colours.backgroundLight;
    final borderColor = isDark ? Colours.borderDark : Colours.borderLight;

    return Container(
      padding: const EdgeInsets.all(UIConfig.terminalPadding),
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: bgColor,
        borderRadius: BorderRadius.circular(UIConfig.terminalRadius),
        border: Border.all(color: borderColor, width: UIConfig.borderThickness),
      ),
      child: Scrollbar(
        thumbVisibility: true,
        child: SingleChildScrollView(
          child: Text(
            lines.join('\n'),
            style: TextStyle(color: textColor, fontFamily: 'Courier New'),
          ),
        ),
      ),
    );
  }
}

class Graph extends StatelessWidget {
  const Graph({
    super.key,
    this.width = 400,
    this.height = 400,
    this.title = const FancyText(
      text: 'Graph Title',
      icon: Icon(FeatherIcons.barChart2),
      fontWeight: FontWeight.bold,
    ),
  });

  final double width, height;
  final FancyText title;

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final borderColor = isDark ? Colours.borderDark : Colours.borderLight;
    final lineColor = isDark
        ? Colours.graphGradientOneDark
        : Colours.graphGradientOneLight;
    final gradientTop = isDark
        ? Colours.graphGradientOneDark
        : Colours.graphGradientOneLight;
    final gradientBottom = isDark
        ? Colours.graphGradientTwoDark
        : Colours.graphGradientTwoLight;

    return Column(
      children: [
        title,
        const SizedBox(height: 10),
        SizedBox(
          width: width,
          height: height,

          child: LineChart(
            LineChartData(
              lineTouchData: LineTouchData(
                handleBuiltInTouches: true,
                touchTooltipData: LineTouchTooltipData(
                  tooltipBorder: BorderSide(
                    color: borderColor,
                    width: UIConfig.borderThickness,
                  ),
                ),
              ),
              titlesData: FlTitlesData(
                leftTitles: AxisTitles(
                  sideTitles: SideTitles(showTitles: false),
                ),
                rightTitles: AxisTitles(
                  sideTitles: SideTitles(showTitles: false),
                ),
                bottomTitles: AxisTitles(
                  sideTitles: SideTitles(showTitles: false),
                ),
                topTitles: AxisTitles(
                  sideTitles: SideTitles(showTitles: false),
                ),
              ),

              gridData: FlGridData(
                show: true,
                drawHorizontalLine: false,
                drawVerticalLine: true,
              ),

              // change grid colour
              borderData: FlBorderData(
                show: true,
                border: Border.all(
                  color: borderColor,
                  width: UIConfig.borderThickness,
                ),
              ),

              lineBarsData: [
                LineChartBarData(
                  spots: const [
                    FlSpot(0, 1),
                    FlSpot(1, 3),
                    FlSpot(2, 2),
                    FlSpot(3, 5),
                    FlSpot(4, 3),
                    FlSpot(5, 4),
                    FlSpot(6, 7),
                  ],
                  dotData: FlDotData(show: false),
                  isCurved: true,
                  barWidth: 3,
                  color: lineColor,
                  belowBarData: BarAreaData(
                    show: true,
                    gradient: LinearGradient(
                      colors: [gradientTop, gradientBottom],
                      begin: Alignment.topCenter,
                      end: Alignment.bottomCenter,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class Toolbar extends StatefulWidget {
  const Toolbar({super.key});

  @override
  State<Toolbar> createState() => IconButtons();
}

/// Use this method to add/remove icons from the toolbar at the top.
class IconButtons extends State<Toolbar> {
  @override
  Widget build(BuildContext context) {
    // Determine the current brightness to show the correct icon
    final brightness = Theme.of(context).brightness;
    final isDark = brightness == Brightness.dark;

    return Container(
      padding: const EdgeInsets.all(4.0),
      decoration: BoxDecoration(
        border: Border.all(
          color: Theme.of(context).colorScheme.outline,
          width: UIConfig.borderThickness,
        ),
        borderRadius: BorderRadius.circular(UIConfig.cornerRadius),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        textDirection: TextDirection.rtl,
        children: <Widget>[
          IconButton.filledTonal(
            onPressed: () async => {await launchUrl(Uri.parse(githubLink))},
            icon: const Icon(FeatherIcons.github),
          ),

          IconButton.filledTonal(
            onPressed: () async => {},
            icon: const Icon(FeatherIcons.settings),
          ),

          const SizedBox(width: 8.0),
          IconButton.filledTonal(
            onPressed: () {
              themeObserver.value = isDark ? ThemeMode.light : ThemeMode.dark;
            },
            icon: Icon(isDark ? FeatherIcons.moon : FeatherIcons.sun),
          ),
        ],
      ),
    );
  }
}
