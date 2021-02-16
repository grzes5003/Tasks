import click


@click.command()
@click.option("-m", "--mode",
              type=click.Choice(['B2D', 'D2B'], case_sensitive=False),
              help="Select the direction of number conversion")
@click.argument('number', type=click.INT)
def convert(mode, number):
    """Program converting NUMBER between Decimal and Binary notation"""
    try:
        if mode == 'B2D':
            result = int(str(number), 2)
        else:
            result = '{0:b}'.format(number)
    except Exception as e:
        raise click.ClickException(e)
    click.echo(result)


if __name__ == '__main__':
    convert()
