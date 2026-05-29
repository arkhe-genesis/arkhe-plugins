import asyncio
import typer
from rich.console import Console

app = typer.Typer()
console = Console()

@app.command()
def hotspots(
    trajectory: str = typer.Argument(..., help="Path to MD trajectory"),
    topology: str = typer.Argument(..., help="Path to topology file"),
    system_name: str = typer.Option("unknown", help="System name"),
    anchor: bool = typer.Option(True, help="Anchor results on TemporalChain"),
):
    """Analyze interaction hotspots in MD trajectory (Substrato 949)."""
    from arkhe.substrates.interaction_hotspots import InteractionHotspotsAnalyzer

    async def _hotspots():
        analyzer = InteractionHotspotsAnalyzer()
        result = await analyzer.analyze_trajectory(
            trajectory, topology, system_name, anchor
        )
        console.print(f"[green]Analysis complete[/green]")
        console.print(f"  Mean log10 deviation: {result.mean_log_deviation:.3f}")
        console.print(f"  Anisotropy index: {result.anisotropy_index:.3f}")
        console.print(f"  Hotspot residue pairs: {len(result.residue_pairs)}")

    asyncio.run(_hotspots())

if __name__ == "__main__":
    app()
