import { Badge } from "@risc0/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@risc0/ui/card";
import { type ChartConfig, ChartContainer, ChartTooltip } from "@risc0/ui/chart";
import { joinWords } from "@risc0/ui/utils/join-words";
import { Area, AreaChart, CartesianGrid, Label, XAxis, YAxis } from "recharts";
import type { FormattedDataSetEntry } from "./collect-benches-per-test-case";

function CustomTooltip({ active, payload }: any) {
  if (active && payload && payload.length) {
    const data = payload[0].payload;
    return (
      <div className="w-[480px] rounded-md bg-black p-2 text-white shadow-2xl">
        <div>
          Commit{" "}
          <Badge className="p-0" variant="secondary">
            {data.commit}
          </Badge>
        </div>
        <br />
        <div className="line-clamp-4">{data.commitMessage}</div>
        <br />
        <div className="text-[11px]">
          {`Commited on ${new Date(data.commitTimestamp).toUTCString()} by`}{" "}
          <Badge className="p-0" variant="secondary">
            @{data.committer}
          </Badge>
        </div>
      </div>
    );
  }

  return null;
}

const chartConfig = {
  chartConfig: {
    label: "chartConfig",
    color: "hsl(var(--primary))",
  },
} satisfies ChartConfig;

export function renderGraph({
  platformName,
  benchName,
  dataset,
}: { platformName: string; benchName: string; dataset: FormattedDataSetEntry[] }) {
  const chartData = dataset.map((entry) => ({
    commit: entry.commit.id.slice(0, 7),
    value: entry.bench.value,
    commitUrl: entry.commit.url,
    commitMessage: entry.commit.message,
    commitTimestamp: entry.commit.timestamp,
    committer: entry.commit.committer.username,
  }));

  return (
    <Card id={`${platformName}-${benchName}`} className="relative w-full border-none bg-transparent pt-8">
      <CardHeader className="pt-0">
        <CardTitle className="flex flex-row items-baseline gap-5">
          <span className="font-mono">{benchName}</span>
          <span className="text-muted-foreground text-xs">{joinWords(platformName)}</span>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <ChartContainer config={chartConfig} className="h-[320px] w-full">
          <AreaChart
            accessibilityLayer
            data={chartData}
            margin={{
              left: 12,
              right: 12,
            }}
          >
            <CartesianGrid />
            <defs>
              <linearGradient id="fill" x1="0" y1="0" x2="0" y2="1">
                <stop offset="5%" stopColor="var(--color-chartConfig)" stopOpacity={1} />
                <stop offset="95%" stopColor="var(--color-chartConfig)" stopOpacity={0.6} />
              </linearGradient>
            </defs>

            <Area
              animationDuration={500}
              dataKey="value"
              type="monotone"
              fill="url(#fill)"
              fillOpacity={0.2}
              stroke="var(--color-chartConfig)"
              activeDot={{
                r: 8,
                cursor: "pointer",
                onClick: (_, { index }: any) => {
                  if (chartData[index]?.commitUrl) {
                    window.open(chartData[index].commitUrl);
                  }
                },
              }}
            />

            <XAxis dataKey="commit" tickLine={false} axisLine={false} />

            <YAxis
              dataKey="value"
              tickLine={false}
              axisLine={false}
              domain={[0, (dataMax) => (dataMax * 1.25).toFixed(0)]}
            >
              <Label angle={270} offset={8} position="left" style={{ textAnchor: "middle" }}>
                Hz
              </Label>
            </YAxis>

            <ChartTooltip cursor={false} content={<CustomTooltip />} />
          </AreaChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
